use super::port::Port;

#[derive(Debug)]
struct CreateInputData {
    // TODO: add data
}

#[derive(Debug)]
struct CreateOutputData {
    // TODO: add data
}

#[derive(Debug)]
struct CreateInteractor {
    // TODO: add repository
}

impl Port<CreateInputData, CreateOutputData> for CreateInteractor {
    fn exec(&self, input: CreateInputData) -> CreateOutputData {
        println!("{:?}", input);
        CreateOutputData{}
    }
}
