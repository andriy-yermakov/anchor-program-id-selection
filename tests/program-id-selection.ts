import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { ProgramIdSelection } from '../target/types/program_id_selection';

describe('program-id-selection', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.ProgramIdSelection as Program<ProgramIdSelection>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
