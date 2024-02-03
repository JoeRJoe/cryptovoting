import "./App.css";
import { PublicKey } from "@solana/web3.js";
import { Program, utils } from "@project-serum/anchor";
import useProvider from "./useProvider";
import useProgram from "./useProgram";

export default function useVotazione() {
  const [getProvider] = useProvider("devnet", "processed");
  const [SystemProgram, programId, idl] = useProgram();

  const createVotazione = async () => {
    try {
      const provider = getProvider();
      const program = new Program(idl, programId, provider);
      const [votazione] = PublicKey.findProgramAddressSync(
        [
          utils.bytes.utf8.encode("Prova"),
          provider.wallet.publicKey.toBuffer(),
        ],
        programId
      );
      await program.methods.createVotazione(
        "Prova",
        "Prova creazione votazione",
        new Date().getTime(),
        {
          accounts: {
            votazione: votazione,
            user: provider.wallet.publicKey,
            systemProgram: SystemProgram.programId,
          }
        }
      ).rpc();
      console.log("Votazione creata");
    } catch (error) {
      console.error(error);
    }
  }

  return [createVotazione];
}
