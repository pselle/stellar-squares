import { useQueryClient } from "@tanstack/react-query";
import squares_gallery from "../contracts/squares_gallery";
import {
  COLLECTION_SYMBOL,
  useGetCollectionAddress,
  useGetGalleryAddress,
  useGetOwner,
  useGetTokenUri,
} from "../hooks/useNftCollection";
import { useWallet } from "../hooks/useWallet";
import styles from "./styles/ArtCard.module.css";

const ArtCard: React.FC<{ tokenId: number }> = ({ tokenId }) => {
  const { data: galleryAddress } = useGetGalleryAddress();
  const { data: collectionAddress, isLoading: isLoadingAddress } =
    useGetCollectionAddress();
  const { data: owner, isLoading: isLoadingOwner } = useGetOwner(
    tokenId,
    collectionAddress,
    galleryAddress,
  );
  const queryClient = useQueryClient();

  const { data: tokenURI, isLoading: isLoadingTokenUri } = useGetTokenUri(
    tokenId,
    collectionAddress,
  );

  const leftPadTokenId = String(tokenId).padStart(2, "0");

  const isLoading = isLoadingAddress || isLoadingOwner;
  const displayOwner = isLoading ? "Loading..." : owner;

  const { address, signTransaction } = useWallet();
  const purchaseNFT = async () => {
    if (!collectionAddress || !address) return;

    const transaction = await squares_gallery.purchase_nft(
      {
        symbol: COLLECTION_SYMBOL,
        token_id: tokenId,
        buyer: address,
        // @ts-expect-error js-stellar-sdk has bad typings; publicKey is, in fact, allowed
      },
      { publicKey: address },
    );

    try {
      const result = await transaction.signAndSend({
        signTransaction: signTransaction,
      });
      // eslint-disable-next-line @typescript-eslint/no-unsafe-enum-comparison
      if (result.getTransactionResponse?.status === "SUCCESS") {
        await queryClient.invalidateQueries({
          queryKey: ["nftOwner", { tokenId }],
        });
        alert("Successfully purchased NFT!");
      }
    } catch (e) {
      console.error("Error purchasing NFT:", e);
      alert("Failed to purchase NFT");
    }
  };

  return (
    <div className={styles.artCard}>
      <h2>Square #{tokenId}</h2>
      <img
        src={`./art/${leftPadTokenId}-squares.png`}
        alt={`art piece #${tokenId}`}
      />
      <div className={styles.artCardInfo}>
        <p>Owned by {displayOwner ?? "(Error loading owner)"}</p>
        {!isLoadingTokenUri && tokenURI && (
          <>
            <p className={styles.tokenUri}>
              <a href={tokenURI} target="_blank" rel="noopener noreferrer">
                Token URI
              </a>
            </p>
            {address && displayOwner == "the Gallery Contract" && (
              <p className={styles.tokenUri}>
                <button
                  onClick={() => {
                    void purchaseNFT();
                  }}
                >
                  Buy NFT
                </button>
              </p>
            )}
          </>
        )}
      </div>
    </div>
  );
};

export default ArtCard;
