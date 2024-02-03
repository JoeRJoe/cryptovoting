import { web3 } from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";
import idl from "../../smart_contract/target/idl/smart_contract.json";

export default function useProgram() {
  const { SystemProgram } = web3;
  const programId = new PublicKey(idl.metadata.address);

  return [SystemProgram, programId, idl];
}
