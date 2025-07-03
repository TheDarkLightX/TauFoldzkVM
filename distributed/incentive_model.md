# Distributed Proving Incentive Model

## Overview

The TauFoldZKVM distributed proving network uses economic incentives to ensure:
1. **Availability**: Provers compete to provide proofs
2. **Correctness**: Verifiers rewarded for finding errors
3. **Efficiency**: Market-based pricing

## Proof Markets

### Request Flow
1. User submits proof request with reward
2. Request split into shards
3. Provers bid on shards
4. Lowest commitment wins (reverse auction)
5. Proofs submitted and aggregated
6. Payment on successful verification

### Shard Assignment
```
Assignment Score = Hash(prover_address, shard_id, bid)
Lowest score wins shard
```

### Pricing Dynamics
- Base reward split among shards
- Bonus for fast completion
- Penalty for failed proofs
- Market determines equilibrium price

## Verification Market

### Verification Incentives
- Anyone can verify submitted proofs
- Reward for finding invalid proofs (slash amount)
- Small fee for valid verification
- Encourages thorough checking

### Slashing Conditions
1. Invalid proof submitted
2. Timeout on assigned shard
3. Malicious behavior detected

## Network Effects

### Positive Feedback Loops
1. More provers → Lower prices → More users
2. More verifiers → Higher security → More trust
3. More volume → Better liquidity → Stable pricing

### Anti-Sybil Measures
- Proof-of-work commitment for bids
- Reputation scoring over time
- Stake requirements for large shards

## Implementation in Tau

All incentive logic encoded as constraints:
- Auction resolution
- Payment distribution  
- Slashing logic
- Reputation updates

This ensures fairness and transparency in the proving market.
