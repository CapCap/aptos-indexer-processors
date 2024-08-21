-- Your SQL goes here

ALTER TABLE swap_events
ADD COLUMN balance_as_fraction_of_circulating_supply_before_q64 NUMERIC NOT NULL,
ADD COLUMN balance_as_fraction_of_circulating_supply_after_q64 NUMERIC NOT NULL;

ALTER TABLE liquidity_events
RENAME COLUMN pro_rata_base_donation_claim_amount TO base_donation_claim_amount;

ALTER TABLE liquidity_events
RENAME COLUMN pro_rata_quote_donation_claim_amount TO quote_donation_claim_amount;

ALTER TABLE user_liquidity_pools
RENAME COLUMN pro_rata_base_donation_claim_amount TO base_donation_claim_amount;

ALTER TABLE user_liquidity_pools
RENAME COLUMN pro_rata_quote_donation_claim_amount TO quote_donation_claim_amount;
