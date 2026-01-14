import {
  useGetCollectionAddress,
  useGetGalleryAddress,
  useGetOwner,
  useGetTokenUri,
} from "../hooks/useNftCollection";
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

  const { data: tokenURI, isLoading: isLoadingTokenUri } = useGetTokenUri(
    tokenId,
    collectionAddress,
  );

  const leftPadTokenId = String(tokenId).padStart(2, "0");

  const isLoading = isLoadingAddress || isLoadingOwner;
  const displayOwner = isLoading ? "Loading..." : owner;

  return (
    <div className={styles.artCard}>
      <h2>Square #{tokenId}</h2>
      <img
        src={`./art/${leftPadTokenId}-squares.png`}
        alt={`art piece #${tokenId}`}
      />
      <div className={styles.artCardInfo}>
        <p>Owned by {displayOwner ?? "Loading..."}</p>
        {!isLoadingTokenUri && tokenURI && (
          <p className={styles.tokenUri}>
            <a href={tokenURI} target="_blank" rel="noopener noreferrer">
              Token URI
            </a>
          </p>
        )}
      </div>
    </div>
  );
};

export default ArtCard;
