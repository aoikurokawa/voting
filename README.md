# Voting

Develop a DAO voting program using Anchor. This program should allow users to vote on proposals and display results. Optionally, implement "privacy" voting using Zero-Knowledge (ZK) proofs or verifiable compute. Reward points should be given to users for participation.

- Create a DAO voting system
- Implement a voting system and display result
- Add privacy voting using ZK proofs or verifiable compute
- Reward points to users for voting participation

## Prepare

Set env variable `ANCHOR_WALLET`

Bash:

```bash
export ANCHOR_WALLET="/home/{username}/.config/solana/id.json"
```

Fish:

```bash
set -x ANCHOR_WALLET "/home/{username}/.config/solana/id.json"
```

## Run the app

### Show all commands

```bash
cargo r -- --help

# Usage: client <COMMAND>
# 
# Commands:
#   create-governance  Create a governance
#   join               Join the governance
#   create-proposal    Create a proposal
#   start-vote         Start the voting
#   commit-vote        Start the voting
#   reveal-vote        Reveal vote
#   help               Print this message or the help of the given subcommand(s)
# 
# Options:
#   -h, --help     Print help
#   -V, --version  Print version
```

### Create a governance

Pass the argument the name of governance

```bash
cargo r -- create-governance 'superteam'
```

### Join the governance

Pass the argument the name of governance

```bash
cargo r -- join 'superteam'
```

### Create a proposal

Pass the argument 
- name: name of governance
- title: Proposal title

```bash
cargo r -- create-proposal 'superteam' 'Hello'
```

### Start voting

Pass the argument 
- name: name of governance
- title: Proposal title
- end time: 1720950304

```bash
cargo r -- start-vote 'superteam' 'Hello' 1720950304
```

### Create a vote commitment

Pass the argument 
- name: name of governance
- title: Proposal title
- vote: Yes => 1, No => 0
- salt: any string you like

```bash
cargo r -- commit-vote 'superteam' 'Hello' 1 'salt'
```

### Reveal vote

Pass the argument 
- name: name of governance
- title: Proposal title
- vote: Yes => 1, No => 0
- salt: any string you like

```bash
cargo r -- reveal-vote 'superteam' 'Hello' 1 'salt'
```

## Reference
