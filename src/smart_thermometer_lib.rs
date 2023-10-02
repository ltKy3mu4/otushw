pub struct SmartThermometer {
    id: String,
    current_temperature: f64,
}

impl SmartThermometer{
    pub fn new(id: String) -> Self {
        return SmartThermometer {
            id,
            current_temperature : 0.0
        };
    }

    pub fn set_temperature(& mut self, temp: f64){
        self.current_temperature = temp;
    }


    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn current_temperature(&self) -> f64 {
        self.current_temperature
    }

}