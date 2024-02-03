import "./App.css";
import useWallet from "./useWallet";
import { Buffer } from "buffer";
import useVotazione from "./useVotazione";

window.Buffer = Buffer;

function App() {
  const [wallet, connectWallet] = useWallet();
  const [createVotazione] = useVotazione(wallet);

  const connectButton = (
    <button onClick={connectWallet}>Connetti Wallet</button>
  );

  const createVotazioneButton = (<button onClick={createVotazione}>Crea Votazione</button>);

  return (
    <>
      {!wallet && connectButton}
      {wallet && createVotazioneButton}

    </>
  );
}

export default App;
