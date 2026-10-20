//! Contract events emitted by the ticketing contract.
#![allow(missing_docs)]

use soroban_sdk::{contractevent, Address, String};

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
/// Emitted when a ticket is minted.
pub struct TicketIssued {
    #[topic]
    pub ticket_id: u64,
    pub event_id: u64,
    pub owner: Address,
    pub tier: String,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
/// Emitted once when the contract is initialized.
pub struct ContractInitialized {
    #[topic]
    pub admin: Address,
    pub payment_token: Address,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
/// Emitted when the admin changes the purchase throttle.
pub struct PurchaseThrottleUpdated {
    #[topic]
    pub admin: Address,
    pub min_ledger_spacing: u32,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
/// Emitted when the admin proposes a payment token change.
pub struct PaymentTokenProposed {
    #[topic]
    pub admin: Address,
    pub new_token: Address,
    pub apply_after_ledger: u32,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
/// Emitted when a proposed payment token change is applied.
pub struct PaymentTokenChanged {
    #[topic]
    pub admin: Address,
    pub old_token: Address,
    pub new_token: Address,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
/// Emitted when a ticket is checked in at the point of entry.
pub struct TicketCheckedIn {
    #[topic]
    pub ticket_id: u64,
    pub organizer: Address,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
/// Emitted when the admin adds an organizer to the event-creation allowlist.
pub struct OrganizerApproved {
    #[topic]
    pub admin: Address,
    pub organizer: Address,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
/// Emitted when the admin removes an organizer from the event-creation allowlist.
pub struct OrganizerRevoked {
    #[topic]
    pub admin: Address,
    pub organizer: Address,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
/// Emitted when the organizer cancels an event (issue #91).
pub struct EventCancelled {
    #[topic]
    pub event_id: u64,
    pub organizer: Address,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
/// Emitted when a ticket changes owner through `transfer_ticket` (issue #115).
pub struct TicketTransferred {
    #[topic]
    pub ticket_id: u64,
    pub from: Address,
    pub to: Address,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
/// Emitted when the admin changes the primary sale platform fee (issue #104).
pub struct PlatformFeeUpdated {
    #[topic]
    pub admin: Address,
    pub fee_bps: u32,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
/// Emitted when an owner starts an auction for a ticket (issue #105).
pub struct AuctionStarted {
    #[topic]
    pub ticket_id: u64,
    pub seller: Address,
    pub reserve_price: i128,
    pub ends_at: u64,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
/// Emitted when a bid becomes the highest bid on an auction (issue #105).
pub struct BidPlaced {
    #[topic]
    pub ticket_id: u64,
    pub bidder: Address,
    pub amount: i128,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
/// Emitted when an auction settles with a winning bid (issue #105).
pub struct AuctionSettled {
    #[topic]
    pub ticket_id: u64,
    pub buyer: Address,
    pub amount: i128,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
/// Emitted when a buyer makes an offer on a listed ticket (issue #106).
pub struct OfferMade {
    #[topic]
    pub ticket_id: u64,
    pub buyer: Address,
    pub amount: i128,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
/// Emitted when an open offer is cancelled and its funds returned (issue #106).
pub struct OfferCancelled {
    #[topic]
    pub ticket_id: u64,
    pub buyer: Address,
    pub amount: i128,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
/// Emitted when the owner accepts an offer and the ticket changes hands (issue #106).
pub struct OfferAccepted {
    #[topic]
    pub ticket_id: u64,
    pub buyer: Address,
    pub amount: i128,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
/// Emitted when an organizer registers a new event (issue #119).
pub struct EventCreated {
    #[topic]
    pub organizer: Address,
    pub event_id: u64,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
/// Emitted when a ticket is sold through `purchase_primary` (issue #118).
pub struct PrimarySale {
    #[topic]
    pub ticket_id: u64,
    pub buyer: Address,
    pub price: i128,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
/// Emitted when a ticket is listed for resale through `list_for_resale` (issue #116).
pub struct TicketListed {
    #[topic]
    pub ticket_id: u64,
    pub owner: Address,
    pub price: i128,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
/// Emitted when a resale listing is cancelled through `cancel_resale` (issue #116).
pub struct ListingCancelled {
    #[topic]
    pub ticket_id: u64,
    pub owner: Address,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
/// Emitted when a resale listing is bought through `buy_resale` (issue #117).
pub struct TicketResold {
    #[topic]
    pub ticket_id: u64,
    pub seller: Address,
    pub buyer: Address,
    pub price: i128,
    pub royalty: i128,
}
