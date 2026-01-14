import { useQuery } from "@tanstack/react-query";
import nft from "../contracts/nft_sequential_minting_example";
import styles from "./styles/ArtCard.module.css";

const useGetOwner = (tokenId: number) =>
  useQuery({
    queryKey: ["nftOwner", { tokenId }],
    queryFn: async () => {
      const transaction = await nft.owner_of({ token_id: tokenId });
      if (typeof transaction.result === "string") {
        return transaction.result;
      }
      // Otherwise, the token has not been minted
      return "Token not minted";
    },
    enabled: true,
  });

const ArtCard: React.FC<{ tokenId: number }> = ({ tokenId }) => {
  const { data: owner } = useGetOwner(tokenId);
  const leftPadTokenId = String(tokenId).padStart(2, "0");

  return (
    <div className={styles.artCard}>
      <h3>
        art piece #{tokenId}, owned by {owner ?? "Loading..."}
        <img
          src={`./art/${leftPadTokenId}-squares.png`}
          alt={`art piece #${tokenId}`}
        />
      </h3>
    </div>
  );
};

export default ArtCard;
