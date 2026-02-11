import React from "react";
import ConnectAccount from "../components/ConnectAccount.tsx";
import { Layout, Text } from "@stellar/design-system";
import ArtCard from "../components/ArtCard";
import styles from "./Home.module.css";

const Home: React.FC = () => {
  const AppName = "Important Diagrams";

  return (
    <>
      <Layout.Header
        projectId={AppName}
        projectTitle={AppName}
        contentRight={
          <>
            <ConnectAccount />
          </>
        }
      />
      <Layout.Content>
        <Layout.Inset>
          <div className={styles.collectionLayout}>
            <div className={styles.collectionHeader}>
              <Text as="h1" size="xl">
                Schotter Squares Collection
              </Text>
              <p className={styles.collectionDescription}>
                A generative art collection featuring geometric squares in the
                style of Georg Nees' Schotter. Each piece is a unique NFT
                showcasing the beauty of controlled randomness and minimalist
                design.
              </p>
            </div>

            <div className={styles.artCardsGrid}>
              {Array.from(Array(20)).map((_, tokenId) => (
                <ArtCard key={`card-${tokenId}`} tokenId={tokenId} />
              ))}
            </div>
          </div>
        </Layout.Inset>
      </Layout.Content>
      <Layout.Footer hideLegalLinks={true}>
        <span>
          © {new Date().getFullYear()} {AppName}. Licensed under the{" "}
          <a
            href="http://www.apache.org/licenses/LICENSE-2.0"
            target="_blank"
            rel="noopener noreferrer"
          >
            Apache License, Version 2.0
          </a>
          .
        </span>
      </Layout.Footer>
    </>
  );
};

export default Home;
