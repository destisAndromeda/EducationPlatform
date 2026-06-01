import { UI } from '../i18n/cs.js';

/** Wallet facade — isolates provider-specific details (Phantom, Solflare, Backpack). */
export async function connectSolanaWallet() {
  const provider = window?.solana;

  if (!provider?.isPhantom) {
    throw new Error(UI.walletNotFound);
  }

  const result = await provider.connect();
  return {
    provider,
    address: result?.publicKey?.toString() || null,
  };
}

export async function disconnectSolanaWallet(provider) {
  if (provider?.disconnect) {
    await provider.disconnect();
  }
}
