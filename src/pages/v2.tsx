import React from "react";
import ArtCard from "../components/ArtCard";
import styles from "./v2.module.css";
import { connectWallet } from "../util/wallet";
import { useWallet } from "../hooks/useWallet";

const Home: React.FC = () => {
  const { address, isPending } = useWallet();
  const buttonLabel = isPending ? "Loading..." : "Connect";

  return (
    <>
      <header>Header content here</header>
      <nav>
        {!address && (
          <button onClick={() => void connectWallet()} disabled={isPending}>
            {buttonLabel}
          </button>
        )}
      </nav>
      <main>
        <div className={styles.artCardsGrid}>
          {Array.from(Array(20)).map((_, tokenId) => (
            <ArtCard key={`card-${tokenId}`} tokenId={tokenId} />
          ))}
        </div>
      </main>
    </>
  );
};

export default Home;
