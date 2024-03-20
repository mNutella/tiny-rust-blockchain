// DNS simple specification
// https://mislove.org/teaching/cs4700/spring11/handouts/project1-primer.pdf
// https://www.perplexity.ai/search/What-are-the-WK5__SJKQ_CxBudg4TniwA

// ~~~~~~~~~~~~~~~ Headers structure ~~~~~~~~~~~~~~~
// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
// |                       ID                      |
// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
// |QR| Opcode |AA|TC|RD|RA| Z |       RCODE       |
// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
// |                   QDCOUNT                     |
// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
// |                   ANCOUNT                     |
// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+

struct DNSHeader {
    // A bit identifier assigned by the program that generates any kind of query.
    // This identifier is copied the corresponding reply and can be used by the requester to match up replies to outstanding queries.
    // Random bit number for each request.
    id: u16,
    flags: u16,
    // The unsigned bit integer specifying the number of entries in the question section.
    qst_count: u16,
    // The unsigned bit integer specifying the number of entries in the answer section.
    ans_count: u16,
}

struct DNSPacket {
    header: DNSHeader,
    qst_sec: [i32; 1024],
    ans_sec: [i32; 1024],
}
