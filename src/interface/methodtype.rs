pub trait Method {}

pub struct CreateMethod {}
pub struct GetMethod {}
pub struct DeleteMethod {}

impl Method for CreateMethod {}
impl Method for GetMethod {}
impl Method for DeleteMethod {}
