use std::collections::HashMap;

struct Guest {
    name : String,
    nights : u32,
}

struct Hotel{
    rooms : HashMap<u32, Guest>,
}

impl Hotel {
    fn new() -> Self {
        Self { rooms: HashMap::new() }
    }

    //Takes ownership of the guest and the room number
    fn check_in(&mut self, room_number: u32, guest: Guest) {
        self.rooms.insert(room_number, guest);
    }

    // Immutable borrow — just viewing the guest
    fn peek_guest(&self, room : u32) -> Option<&Guest> {
        self.rooms.get(&room)
    }

    // Mutable borrow — modifying the guest
    fn extend_stay(&mut self, room : u32, extra_nights: u32) -> Option<&Guest> {
        if let Some(guest) = self.rooms.get_mut(&room) {
            guest.nights += extra_nights;
            Some(guest)
        } else {
            None
        }
    }

     // Borrowed reference with a lifetime
    fn guest_report(&self, room: u32) -> Option<&str> {
        self.rooms.get(&room).map(|g| g.name.as_str())
    }

}

fn main() {
    let mut hotel = Hotel::new();

    // Ownership moves here
    hotel.check_in(101, Guest {
        name: String::from("Alice"),
        nights: 3,
    });

    hotel.check_in(102, Guest {
        name: String::from("Bob"),
        nights: 2,
    });

    // Immutable borrow
    if let Some(guest) = hotel.peek_guest(101) {
        println!("Room 101: {} staying for {} nights", guest.name, guest.nights);
    }

    // Mutable borrow
    hotel.extend_stay(101, 2);

    // Borrow with explicit lifetime
    if let Some(name) = hotel.guest_report(101) {
        println!("Guest in room 101 is still: {}", name);
    }

    // Move attempt (illegal - try uncommenting)
    // let moved_guest = hotel.rooms.remove(&102).unwrap();
    // println!("Moved out: {}", moved_guest.name);
}

