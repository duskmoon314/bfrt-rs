#include <core.p4>
#include <tna.p4>

#include "common/headers.p4"
#include "common/util.p4"

struct metadata_t {}

parser SwitchIngressParser(
    packet_in pkt,
    out header_t hdr,
    out metadata_t ig_md,
    out ingress_intrinsic_metadata_t ig_intr_md
) {
    TofinoIngressParser() tofino_parser;

    state start {
        tofino_parser.apply(pkt, ig_intr_md);
        transition parse_ethernet;
    }

    state parse_ethernet {
        pkt.extract(hdr.ethernet);
        transition select (hdr.ethernet.ether_type) {
            ETHERTYPE_IPV4 : parse_ipv4;
            default : reject;
        }
    }

    state parse_ipv4 {
        pkt.extract(hdr.ipv4);
        transition accept;
    }
}

control SwitchIngressDeparser(
    packet_out pkt,
    inout header_t hdr,
    in metadata_t ig_md,
    in ingress_intrinsic_metadata_for_deparser_t ig_intr_dprsr_md
) {
    apply {
        pkt.emit(hdr);
    }
}

control SwitchIngress(
    inout header_t hdr,
    inout metadata_t ig_md,
    in ingress_intrinsic_metadata_t ig_intr_md,
    in ingress_intrinsic_metadata_from_parser_t in_intr_prsr_md,
    inout ingress_intrinsic_metadata_for_deparser_t ig_intr_dprsr_md,
    inout ingress_intrinsic_metadata_for_tm_t ig_intr_tm_md
) {
    // Direct Counter
    DirectCounter<bit<32>>(CounterType_t.PACKETS_AND_BYTES) direct_counter;
    DirectCounter<bit<32>>(CounterType_t.PACKETS_AND_BYTES) direct_counter2;

    action hit(PortId_t port) {
        direct_counter.count();
        ig_intr_tm_md.ucast_egress_port = port;
    }

    action miss() {
        direct_counter.count();
        ig_intr_dprsr_md.drop_ctl = 0x1;
    }

    table forward {
        key = {
            hdr.ipv4.dst_addr : lpm;
        }

        actions = {
            hit;
            miss;
        }

        size = 1024;
        const default_action = miss;
        counters = direct_counter;
    }

    apply {
        forward.apply();

        ig_intr_tm_md.bypass_egress = 1w1;
    }
}

Pipeline(
    SwitchIngressParser(),
    SwitchIngress(),
    SwitchIngressDeparser(),
    EmptyEgressParser(),
    EmptyEgress(),
    EmptyEgressDeparser()
) pipe;

Switch(pipe) main;