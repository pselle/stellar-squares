import { useQuery, UseQueryResult } from "@tanstack/react-query";
import squaresGallery from "../contracts/squares_gallery";
import * as NftContract from "nft_sequential_minting_example";
import { rpcUrl, networkPassphrase } from "../contracts/util";

// Create a single NFT client instance once we have the collection address
let nftClient: NftContract.Client | null = null;

export const getNftClient = (collectionAddress: string): NftContract.Client => {
  if (!nftClient) {
    nftClient = new NftContract.Client({
      networkPassphrase,
      contractId: collectionAddress,
      rpcUrl,
      allowHttp: true,
      publicKey: undefined,
    });
  }
  return nftClient;
};

// Cache the collection address permanently (it never changes)
export const useGetCollectionAddress = (): UseQueryResult<string, Error> =>
  useQuery({
    queryKey: ["collectionAddress"],
    queryFn: async () => {
      const transaction = await squaresGallery.collection_address();
      if (typeof transaction.result === "string") {
        return transaction.result;
      }
      throw new Error("Failed to get collection address");
    },
    staleTime: Infinity, // Never goes stale - collection address is immutable
    gcTime: Infinity, // Keep in memory forever
    enabled: true,
  });

export const useGetGalleryAddress = (): UseQueryResult<string, Error> =>
  useQuery({
    queryKey: ["galleryAddress"],
    queryFn: async () => {
      const transaction = await squaresGallery.gallery_address();
      if (typeof transaction.result === "string") {
        return transaction.result;
      }
      throw new Error("Failed to get gallery address");
    },
    staleTime: Infinity, // Never goes stale - gallery address is immutable
    gcTime: Infinity, // Keep in memory forever
    enabled: true,
  });

// Cache NFT owners using the shared collection client
export const useGetOwner = (
  tokenId: number,
  collectionAddress?: string,
  galleryAddress?: string,
): UseQueryResult<string, Error> =>
  useQuery({
    queryKey: ["nftOwner", { tokenId }],
    queryFn: async () => {
      if (!collectionAddress) {
        throw new Error("Collection address not available");
      }

      const client = getNftClient(collectionAddress);
      const transaction = await client.owner_of({ token_id: tokenId });

      if (typeof transaction.result === "string") {
        if (transaction.result === galleryAddress) {
          return "the Gallery Contract";
        }
        return transaction.result;
      }
      return "Token not minted";
    },
    enabled: !!collectionAddress && !!galleryAddress,
    staleTime: 1000 * 60 * 5, // Cache for 5 minutes
    gcTime: 1000 * 60 * 30, // Keep in memory for 30 minutes
  });

// Hook to get token URI for metadata
export const useGetTokenUri = (
  tokenId: number,
  collectionAddress?: string,
): UseQueryResult<string | null, Error> =>
  useQuery({
    queryKey: ["tokenUri", { tokenId }],
    queryFn: async () => {
      if (!collectionAddress) {
        throw new Error("Collection address not available");
      }

      const client = getNftClient(collectionAddress);
      const transaction = await client.token_uri({ token_id: tokenId });
      if (typeof transaction.result === "string") {
        return transaction.result;
      }
      return null;
    },
    enabled: !!collectionAddress,
    staleTime: Infinity, // Token URI should never change
    gcTime: 1000 * 60 * 60, // Keep in memory for 1 hour
  });
