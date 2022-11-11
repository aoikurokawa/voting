use std::collections::{HashMap, VecDeque};

pub trait Mediator {
    fn notify_about_arrival(&mut self, train_name: &str) -> bool;
    fn notify_about_departure(&mut self, train_name: &str);
}

pub trait Train {
    fn name(&self) -> &String;
    fn arrive(&mut self, mediator: &mut dyn Mediator);
    fn depart(&mut self, mediator: &mut dyn Mediator);
}

#[derive(Default)]
pub struct TrainStation {
    trains: HashMap<String, Box<dyn Train>>,
    train_queue: VecDeque<String>,
    train_on_platform: Option<String>,
}

impl Mediator for TrainStation {
    fn notify_about_arrival(&mut self, train_name: &str) -> bool {
        if self.train_on_platform.is_some() {
            self.train_queue.push_back(train_name.into());
            false
        } else {
            self.train_on_platform.replace(train_name.into());
            true
        }
    }

    fn notify_about_departure(&mut self, train_name: &str) {
        if Some(train_name.into()) == self.train_on_platform {
            self.train_on_platform = None;

            if let Some(next_train_name) = self.train_queue.pop_front() {
                let mut next_train = self.trains.remove(&next_train_name).unwrap();
                next_train.arrive(self);
                self.trains.insert(next_train_name.clone(), next_train);

                self.train_on_platform = Some(next_train_name);
            }
        }
    }
}

impl TrainStation {
    pub fn accept(&mut self, mut train: impl Train + 'static) {
        if self.trains.contains_key(train.name()) {
            println!("{} has already arrived", train.name());
            return;
        }

        train.arrive(self);
        self.trains.insert(train.name().clone(), Box::new(train));
    }

    pub fn depart(&mut self, name: &'static str) {
        let train = self.trains.remove(name);
        if let Some(mut train) = train {
            train.depart(self);
        } else {
            println!("'{}' is not on the station!", name);
        }
    }
}

pub struct FreightTrain {
    name: String,
}

impl FreightTrain {
    pub fn new(name: &'static str) -> Self {
        Self { name: name.into() }
    }
}

impl Train for FreightTrain {
    fn name(&self) -> &String {
        &self.name
    }

    fn arrive(&mut self, mediator: &mut dyn Mediator) {
        if !mediator.notify_about_arrival(&self.name) {
            println!("FreightTrain {}: Arrival blocked, waiting", self.name);
            return;
        }

        println!("Freight train {}: Arrived", self.name);
    }

    fn depart(&mut self, mediator: &mut dyn Mediator) {
        println!("Freight train {}: Leaving", self.name);
        mediator.notify_about_departure(&self.name);
    }
}

pub struct PassengerTrain {
    name: String,
}

impl PassengerTrain {
    pub fn new(name: &'static str) -> Self {
        Self { name: name.into() }
    }
}

impl Train for PassengerTrain {
    fn name(&self) -> &String {
        &self.name
    }

    fn arrive(&mut self, mediator: &mut dyn Mediator) {
        if !mediator.notify_about_arrival(&self.name) {
            println!("PassengerTrain {}: Arrival blocked, waiting", self.name);
            return;
        }

        println!("Passenger train {}: Arrived", self.name);
    }

    fn depart(&mut self, mediator: &mut dyn Mediator) {
        println!("Passenger train {}: Leaving", self.name);
        mediator.notify_about_departure(&self.name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mediator() {
        let train1 = PassengerTrain::new("Train 1");
        let train2 = PassengerTrain::new("Train 2");

        let mut station = TrainStation::default();

        station.accept(train1);
        station.accept(train2);

        station.depart("Train 1");
        station.depart("Train 2");
        station.depart("Train 3");
    }
}
