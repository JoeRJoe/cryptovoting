import "./App.css";
import { clusterApiUrl, Connection } from "@solana/web3.js";
import { AnchorProvider } from "@project-serum/anchor";

export default function useProvider(networkType, preflightCommitment) {
  const network = clusterApiUrl(networkType);
  const opts = { preflightCommitment: preflightCommitment };
  const getProvider = () => {
    const connection = new Connection(network, opts.preflightCommitment);
    const provider = new AnchorProvider(
      connection,
      window.solana,
      opts.preflightCommitment
    );
    return provider;
  };

  return [getProvider];
}
