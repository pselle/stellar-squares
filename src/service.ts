const IPFS_GATEWAY = "https://ipfs.io/ipfs/";

export function ipfsToHttp(ipfsUri: string): string {
  if (ipfsUri.startsWith("ipfs://")) {
    return IPFS_GATEWAY + ipfsUri.slice(7);
  }
  return ipfsUri;
}

export interface NFTAttribute {
  trait_type: string;
  value: string | number;
}

export interface NFTMetadata {
  name?: string;
  description?: string;
  image?: string;
  attributes?: NFTAttribute[];
  [key: string]: unknown;
}

export async function fetchNFTMetadata(ipfsUri: string): Promise<string> {
  const url = ipfsToHttp(ipfsUri);
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`Failed to fetch metadata: ${response.status}`);
  }
  return JSON.stringify(response.json());
}
