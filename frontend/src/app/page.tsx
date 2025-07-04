'use client';

import { TonConnectButton, TonConnectUIProvider } from '@tonconnect/ui-react';
import { useTonConnectUI } from '@tonconnect/ui-react';
import { useState, useEffect } from 'react';

function WalletConnection() {
  const [tonConnectUI] = useTonConnectUI();
  const [signature, setSignature] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [isClient, setIsClient] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [walletInfo, setWalletInfo] = useState<{account?: {address: string}, device?: {appName: string}} | null>(null);

  useEffect(() => {
    setIsClient(true);
  }, []);

  useEffect(() => {
    if (tonConnectUI) {
      const unsubscribe = tonConnectUI.onStatusChange((walletInfo) => {
        console.log('Wallet status changed:', walletInfo);
        setWalletInfo(walletInfo);
        setError(null);
      });

      return () => {
        unsubscribe();
      };
    }
  }, [tonConnectUI]);

  const handleSignMessage = async () => {
    if (!tonConnectUI.connected) {
      setError('Wallet not connected');
      return;
    }

    setIsLoading(true);
    setError(null);
    try {
      const result = await tonConnectUI.signData({
        type: 'text',
        text: 'I confirm creating a Solana smart wallet through TON signature.',
      });
      
      setSignature(result.signature);
    } catch (error: unknown) {
      console.error('Signing failed:', error);
      setError(error instanceof Error ? error.message : 'Failed to sign message');
    } finally {
      setIsLoading(false);
    }
  };

  const handleDisconnect = () => {
    tonConnectUI.disconnect();
    setSignature(null);
    setError(null);
    setWalletInfo(null);
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 dark:from-gray-900 dark:to-gray-800 flex items-center justify-center p-4">
      <div className="bg-white dark:bg-gray-800 rounded-2xl shadow-xl p-8 max-w-md w-full">
        <div className="text-center mb-8">
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white mb-2">
            TON-Solana Smart Wallet
          </h1>
          <p className="text-gray-600 dark:text-gray-300">
            Connect your TON wallet to create a Solana smart wallet
          </p>
        </div>

        <div className="space-y-6">
          <div className="flex justify-center">
            <TonConnectButton />
          </div>

          {isClient && (
            <div className="bg-gray-100 dark:bg-gray-700 rounded-lg p-3 text-xs">
              <p className="text-gray-600 dark:text-gray-300">
                Debug: Connected = {tonConnectUI.connected ? 'Yes' : 'No'}
              </p>
              {walletInfo && (
                <p className="text-gray-600 dark:text-gray-300">
                  Wallet: {walletInfo.device?.appName || 'Unknown'}
                </p>
              )}
            </div>
          )}

          {isClient && tonConnectUI.connected && (
            <div className="space-y-4">
              <div className="bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-lg p-4">
                <p className="text-green-800 dark:text-green-200 text-sm">
                  ✅ TON wallet connected successfully!
                </p>
                {walletInfo && (
                  <p className="text-green-700 dark:text-green-300 text-xs mt-1">
                    Address: {walletInfo.account?.address ? 
                      `${walletInfo.account.address.slice(0, 8)}...${walletInfo.account.address.slice(-6)}` 
                      : 'Loading...'}
                  </p>
                )}
              </div>

              <div className="flex gap-2">
                <button
                  onClick={handleSignMessage}
                  disabled={isLoading}
                  className="flex-1 bg-blue-600 hover:bg-blue-700 disabled:bg-blue-400 text-white font-medium py-3 px-4 rounded-lg transition-colors"
                >
                  {isLoading ? 'Signing...' : 'Sign Message'}
                </button>
                <button
                  onClick={handleDisconnect}
                  className="px-4 py-3 bg-gray-200 hover:bg-gray-300 dark:bg-gray-600 dark:hover:bg-gray-500 text-gray-800 dark:text-gray-200 rounded-lg transition-colors"
                >
                  Disconnect
                </button>
              </div>

              {signature && (
                <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-4">
                  <p className="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                    Signature:
                  </p>
                  <p className="text-xs text-gray-600 dark:text-gray-400 break-all font-mono">
                    {signature}
                  </p>
                </div>
              )}
            </div>
          )}

          {error && (
            <div className="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-4">
              <p className="text-red-800 dark:text-red-200 text-sm">
                ❌ {error}
              </p>
            </div>
          )}

          {isClient && !tonConnectUI.connected && (
            <div className="bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-lg p-4">
              <p className="text-yellow-800 dark:text-yellow-200 text-sm">
                Please connect your TON wallet to continue
              </p>
              <p className="text-yellow-700 dark:text-yellow-300 text-xs mt-2">
                💡 If connection freezes, try refreshing the page or using a different wallet
              </p>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

export default function Home() {
  return (
    <TonConnectUIProvider manifestUrl="https://ton-connect.github.io/demo-dapp-with-wallet/tonconnect-manifest.json">
      <WalletConnection />
    </TonConnectUIProvider>
  );
}
