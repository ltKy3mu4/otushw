pub struct SmartSocket{
    id: String,
    description: String,
    current_power: f64,
    state: bool,
}

impl SmartSocket {

    pub fn new(id: String, description: String) -> Self {
        return SmartSocket {
            id, 
            description,
            current_power: 0.0,
            state: false
        };
    }

    pub fn get_description(&self) -> &String{
        return &self.description;
    }

    pub fn power(&self) -> f64 {
        return self.current_power;
    }

    pub fn set_power(& mut self, power: f64) {
        self.current_power = power;
    }

    pub fn switch_on(& mut self) {
        self.state = true;
    }

    pub fn switch_off(& mut self){
        self.state = false;
    }


    pub fn state(&self) -> bool {
        self.state
    }


    pub fn id(&self) -> &str {
        &self.id
    }
}