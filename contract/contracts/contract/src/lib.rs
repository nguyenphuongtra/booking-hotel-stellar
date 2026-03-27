#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol, Map};

// Key lưu booking
const BOOKING: Symbol = symbol_short!("BOOK");

#[contract]
pub struct HotelBookingContract;

#[contractimpl]
impl HotelBookingContract {

    // Tạo booking
    pub fn create_booking(env: Env, booking_id: u32, amount: i128) {
        let mut bookings: Map<u32, (i128, Symbol)> =
            env.storage().instance().get(&BOOKING).unwrap_or(Map::new(&env));

        // status = PENDING
        bookings.set(booking_id, (amount, symbol_short!("PENDING")));

        env.storage().instance().set(&BOOKING, &bookings);
    }

    // Thanh toán booking
    pub fn pay_booking(env: Env, booking_id: u32) {
        let mut bookings: Map<u32, (i128, Symbol)> =
            env.storage().instance().get(&BOOKING).unwrap();

        let (amount, _status) = bookings.get(booking_id).unwrap();

        // update status = PAID
        bookings.set(booking_id, (amount, symbol_short!("PAID")));

        env.storage().instance().set(&BOOKING, &bookings);
    }

    // Lấy thông tin booking
    pub fn get_booking(env: Env, booking_id: u32) -> (i128, Symbol) {
        let bookings: Map<u32, (i128, Symbol)> =
            env.storage().instance().get(&BOOKING).unwrap();

        bookings.get(booking_id).unwrap()
    }
}