static mut LIST_ID: Vec<u16> = vec![];

pub struct DnsHeader {
    id: u16,
    qr: bool,
    opcode: bool,
    aa: bool,
    tc: bool,
    rd: bool,
    ra: bool,
    z: bool,
    rcode: u16,
    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16,
}

impl DnsHeader {
    pub(crate) fn new(qr: bool, aa: bool, tc: bool, ra: bool, rcode: u16, qdcount: u16, ancount: u16, nscount: u16, arcount: u16) -> Self {
        use rand::Rng;
        if tc{panic!("TC detected")};
        //attribution d'un id unique et aléatoire
        let mut temp: u16 = rand::thread_rng().gen();
        unsafe {
            while LIST_ID.contains(&temp) {
                temp = rand::thread_rng().gen();
            }
        }

        DnsHeader {
            id: temp,
            qr,
            opcode: false,
            aa,
            tc,
            rd: true,
            ra,
            z: false,
            rcode,
            qdcount,
            ancount,
            nscount,
            arcount,
        }
    }

    //get set
    pub fn id(&self) -> u16 {
        self.id
    }
    pub fn qr(&self) -> bool {
        self.qr
    }
    pub fn opcode(&self) -> bool {
        self.opcode
    }
    pub fn aa(&self) -> bool {
        self.aa
    }
    pub fn tc(&self) -> bool {
        self.tc
    }
    pub fn rd(&self) -> bool {
        self.rd
    }
    pub fn ra(&self) -> bool {
        self.ra
    }
    pub fn z(&self) -> bool {
        self.z
    }
    pub fn rcode(&self) -> u16 {
        self.rcode
    }
    pub fn qdcount(&self) -> u16 {
        self.qdcount
    }
    pub fn ancount(&self) -> u16 {
        self.ancount
    }
    pub fn nscount(&self) -> u16 {
        self.nscount
    }
    pub fn arcount(&self) -> u16 {
        self.arcount
    }
    pub fn set_id(&mut self, id: u16) {
        self.id = id;
    }
    pub fn set_qr(&mut self, qr: bool) {
        self.qr = qr;
    }
    pub fn set_opcode(&mut self, opcode: bool) {
        self.opcode = opcode;
    }
    pub fn set_aa(&mut self, aa: bool) {
        self.aa = aa;
    }
    pub fn set_tc(&mut self, tc: bool) {
        self.tc = tc;
    }
    pub fn set_rd(&mut self, rd: bool) {
        self.rd = rd;
    }
    pub fn set_ra(&mut self, ra: bool) {
        self.ra = ra;
    }
    pub fn set_z(&mut self, z: bool) {
        self.z = z;
    }
    pub fn set_rcode(&mut self, rcode: u16) {
        self.rcode = rcode;
    }
    pub fn set_qdcount(&mut self, qdcount: u16) {
        self.qdcount = qdcount;
    }
    pub fn set_ancount(&mut self, ancount: u16) {
        self.ancount = ancount;
    }
    pub fn set_nscount(&mut self, nscount: u16) {
        self.nscount = nscount;
    }
    pub fn set_arcount(&mut self, arcount: u16) {
        self.arcount = arcount;
    }
}