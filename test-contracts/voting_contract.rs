use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, Symbol, Bytes};

#[contract]
pub struct VotingContract;

#[contracttype]
pub enum DataKey {
    Proposal(u64),
    Vote(u64, Address),
    ProposalCount,
    VotingPeriod,
}

#[contracttype]
pub struct Proposal {
    pub id: u64,
    pub title: Symbol,
    pub description: Bytes,
    pub creator: Address,
    pub votes_for: u64,
    pub votes_against: u64,
    pub end_time: u64,
    pub executed: bool,
}

#[contracttype]
pub enum Vote {
    For,
    Against,
}

#[contractimpl]
impl VotingContract {
    pub fn create_proposal(env: Env, creator: Address, title: Symbol, description: Bytes, voting_period: u64) -> u64 {
        creator.require_auth();
        
        let proposal_count: u64 = env.storage().instance().get(&DataKey::ProposalCount).unwrap_or(0);
        let proposal_id = proposal_count + 1;
        
        let proposal = Proposal {
            id: proposal_id,
            title,
            description,
            creator,
            votes_for: 0,
            votes_against: 0,
            end_time: env.ledger().timestamp() + voting_period,
            executed: false,
        };
        
        env.storage().persistent().set(&DataKey::Proposal(proposal_id), &proposal);
        env.storage().instance().set(&DataKey::ProposalCount, &proposal_id);
        
        proposal_id
    }

    pub fn vote(env: Env, voter: Address, proposal_id: u64, vote: Vote) {
        voter.require_auth();
        
        let mut proposal: Proposal = env.storage().persistent().get(&DataKey::Proposal(proposal_id)).unwrap();
        
        // Check if voting period has ended
        if env.ledger().timestamp() > proposal.end_time {
            panic!("Voting period has ended");
        }
        
        // Check if user has already voted
        if env.storage().persistent().has(&DataKey::Vote(proposal_id, voter.clone())) {
            panic!("User has already voted");
        }
        
        // Record the vote
        env.storage().persistent().set(&DataKey::Vote(proposal_id, voter), &vote);
        
        // Update proposal vote counts
        match vote {
            Vote::For => proposal.votes_for += 1,
            Vote::Against => proposal.votes_against += 1,
        }
        
        env.storage().persistent().set(&DataKey::Proposal(proposal_id), &proposal);
    }

    pub fn get_proposal(env: Env, proposal_id: u64) -> Proposal {
        env.storage().persistent().get(&DataKey::Proposal(proposal_id)).unwrap()
    }

    pub fn execute_proposal(env: Env, proposal_id: u64) {
        let mut proposal: Proposal = env.storage().persistent().get(&DataKey::Proposal(proposal_id)).unwrap();
        
        // Check if voting period has ended
        if env.ledger().timestamp() <= proposal.end_time {
            panic!("Voting period has not ended");
        }
        
        // Check if proposal has already been executed
        if proposal.executed {
            panic!("Proposal has already been executed");
        }
        
        // Check if proposal passed
        if proposal.votes_for <= proposal.votes_against {
            panic!("Proposal did not pass");
        }
        
        proposal.executed = true;
        env.storage().persistent().set(&DataKey::Proposal(proposal_id), &proposal);
        
        // Execute proposal logic here
    }
}
