# SolanaAuditorTraineeInterview

This repository has got 2 parts. The original task for both parts can be found in ___task.md___ file.

# Rust assignment

### Rust version:

#### installed toolchains

- stable-x86_64-unknown-linux-gnu (default)
- bpf

#### active toolchain

- stable-x86_64-unknown-linux-gnu (default)
- rustc 1.62.1 (e092d0b6b 2022-07-16)

### Setup:

```
rustc rust-assignment.rs
./rust-assignment
```

# Analysis of the Solana hack:

### Background Overview

Wormhole is a decentralized, cross-chain message passing protocol. It enables applications to send messages from one chain to another.

On Feb 2, 2022, Wormhole bridge utilizing the Solana and Ethereum blockchain was hacked, resulting in inflating the wETH (Wormhole-wrapped Ether) tokens on Solana side of the bridge. There was a total of 120k wETH minted without being backed by the real ETH. 93k wETH was then bridged to Ethereum blockchain.

### Actual cause of the hack

Wormhole bridge uses a decentralized Guardian voting system which is responsible for signing off transfers between Solana and Ethereum blockchains.

The method responsible for checking the transfers between the chains, called **post_VAA**, calls a **verify_signatures** method to validate the source chain message before minting wETH. This **verify_signatures** method relies on information given by the **instruction sysvar account**.

Each program is responsible for validating all accounts passed to their methods.

However the **verify_signatures** method did not verify, whether the user passed the correct 'instruction sysvar account' program. The attacker could deploy his own program with the same data set, and pass it to the method instead, to bypass the verification. The method was then fooled to think that the fake account was the real one and the signatures coming from it were real.

### Sources

- https://wormholecrypto.medium.com/wormhole-incident-report-02-02-22-ad9b8f21eec6
- https://youtu.be/9YIZ_tpqGPs
- https://www.theverge.com/2022/2/3/22916111/wormhole-hack-github-error-325-million-theft-ethereum-solana
- https://twitter.com/wormholecrypto/status/1489001949881978883?ref_src=twsrc%5Etfw%7Ctwcamp%5Etweetembed%7Ctwterm%5E1489001949881978883%7Ctwgr%5E21a12c5a59af6b59a04624118e8a9a24ab8edae8%7Ctwcon%5Es1_&ref_url=https%3A%2F%2Fwww.theverge.com%2F2022%2F2%2F3%2F22916111%2Fwormhole-hack-github-error-325-million-theft-ethereum-solana
