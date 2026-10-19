//! Platform fee on primary sales (issue #104).
//!
//! The fee is taken from each primary sale and paid to the admin. The organizer
//! receives the rest, through escrow when the event uses it.

use super::*;

/// Prices tier "GA" on event 1 at `price` and mints the buyer enough to pay it.
fn priced_event_and_buyer(
    env: &Env,
    client: &TicketingContractClient,
    token_asset: &StellarAssetClient,
    organizer: &Address,
    price: i128,
) -> Address {
    make_event(env, client, organizer, 1);
    client.set_tier_price(organizer, &1, &String::from_str(env, "GA"), &price);
    let buyer = Address::generate(env);
    mint_tokens(token_asset, &buyer, price);
    buyer
}

fn buy_ga(env: &Env, client: &TicketingContractClient, buyer: &Address) -> u64 {
    client.purchase_primary(
        buyer,
        &1,
        &String::from_str(env, "GA"),
        &String::from_str(env, "unassigned"),
    )
}

#[test]
fn primary_sale_without_a_fee_pays_the_organizer_in_full() {
    let (env, client, token, token_asset, admin, organizer) = setup();
    let buyer = priced_event_and_buyer(&env, &client, &token_asset, &organizer, 1_000);

    assert_eq!(client.platform_fee_bps(), 0);
    buy_ga(&env, &client, &buyer);

    assert_eq!(token.balance(&organizer), 1_000);
    assert_eq!(token.balance(&admin), 0);
    assert_eq!(token.balance(&buyer), 0);
}

#[test]
fn primary_sale_pays_the_fee_to_the_admin_and_the_rest_to_the_organizer() {
    let (env, client, token, token_asset, admin, organizer) = setup();
    let buyer = priced_event_and_buyer(&env, &client, &token_asset, &organizer, 1_000);

    client.set_platform_fee_bps(&admin, &250); // 2.5%
    assert_eq!(client.platform_fee_bps(), 250);
    buy_ga(&env, &client, &buyer);

    assert_eq!(token.balance(&admin), 25);
    assert_eq!(token.balance(&organizer), 975);
    assert_eq!(token.balance(&buyer), 0);
}

#[test]
fn fee_rounds_down_in_the_organizers_favour() {
    let (env, client, token, token_asset, admin, organizer) = setup();
    let buyer = priced_event_and_buyer(&env, &client, &token_asset, &organizer, 1_000);

    // 333 bps of 1_000 is 33.3, which rounds down to 33 for the admin.
    client.set_platform_fee_bps(&admin, &333);
    buy_ga(&env, &client, &buyer);

    assert_eq!(token.balance(&admin), 33);
    assert_eq!(token.balance(&organizer), 967);
}

#[test]
fn setting_the_fee_back_to_zero_turns_it_off() {
    let (env, client, token, token_asset, admin, organizer) = setup();
    let buyer = priced_event_and_buyer(&env, &client, &token_asset, &organizer, 1_000);

    client.set_platform_fee_bps(&admin, &250);
    client.set_platform_fee_bps(&admin, &0);
    buy_ga(&env, &client, &buyer);

    assert_eq!(client.platform_fee_bps(), 0);
    assert_eq!(token.balance(&admin), 0);
    assert_eq!(token.balance(&organizer), 1_000);
}

#[test]
fn a_fee_above_100_percent_is_rejected() {
    let (env, client, _token, _token_asset, admin, _organizer) = setup();

    assert_eq!(
        client.try_set_platform_fee_bps(&admin, &10_001),
        Err(Ok(Error::InvalidPlatformFee))
    );
    assert_eq!(client.platform_fee_bps(), 0);
}

#[test]
fn a_fee_of_100_percent_sends_the_whole_sale_to_the_admin() {
    let (env, client, token, token_asset, admin, organizer) = setup();
    let buyer = priced_event_and_buyer(&env, &client, &token_asset, &organizer, 1_000);

    client.set_platform_fee_bps(&admin, &10_000);
    buy_ga(&env, &client, &buyer);

    assert_eq!(token.balance(&admin), 1_000);
    assert_eq!(token.balance(&organizer), 0);
}

#[test]
fn only_the_admin_can_set_the_platform_fee() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();

    assert_eq!(
        client.try_set_platform_fee_bps(&organizer, &100),
        Err(Ok(Error::NotAdmin))
    );
    assert_eq!(client.platform_fee_bps(), 0);
}

#[test]
fn escrowed_sale_holds_only_the_organizers_share_until_release() {
    let (env, client, token, token_asset, admin, organizer) = setup();
    let buyer = priced_event_and_buyer(&env, &client, &token_asset, &organizer, 1_000);
    client.enable_escrow(&organizer, &1, &500);
    client.set_platform_fee_bps(&admin, &250);

    buy_ga(&env, &client, &buyer);
    // The admin's fee is paid straight out, so escrow holds only the organizer's 975.
    assert_eq!(token.balance(&admin), 25);
    assert_eq!(token.balance(&client.address), 975);
    assert_eq!(token.balance(&organizer), 0);

    env.ledger().set_sequence_number(500);
    client.release_escrow(&organizer, &1);

    assert_eq!(token.balance(&client.address), 0);
    assert_eq!(token.balance(&organizer), 975);
}
