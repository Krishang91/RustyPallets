pub struct Block<Header, Extrinsics> {
    pub header: Header,
    pub extrinsics: Vec<Extrinsics>,
}

pub struct Header<BlockNumber> {
    pub block_number: BlockNumber,
}

pub struct Extrinsic<Caller, Call> {
    pub caller: Caller,
    pub call: Call,
}

pub type DispatchResult = Result<(), &'static str>;

pub trait Dispatch {
    type Caller;
    type Call;
    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult;
}
