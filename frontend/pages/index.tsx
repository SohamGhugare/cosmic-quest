import { useState } from "react";
import { Box, Divider, Text } from "@interchain-ui/react";
import { Layout, Wallet } from "@/components";
import { Hero } from "@/components/Hero";

export default function Home() {
  const [showWallet, setShowWallet] = useState(false);
  const [walletConnected, setWalletConnected] = useState(false);
  const [loading, setLoading] = useState(false);

  const handleWalletConnect = () => {
    setLoading(true);
    setTimeout(() => {
      setWalletConnected(true);
      setLoading(false);
    }, 2000); 
  };

  return (
    <Layout>
      {showWallet ? (
        <>
        <Box marginBottom={"$15"}>
            <Wallet onConnect={handleWalletConnect} />
            {loading && (
              <Text fontSize="$lg" color="blue" textAlign={"center"}>
                Verifying...
              </Text>
            )}
            {walletConnected && !loading && (
              <Text fontSize="$lg" color="green" textAlign={"center"}>
                Congrats! You just completed your first quest!
              </Text>
            )}
          </Box>
          <Divider mb="$16" />
        </>

      ) : (
        <Box marginBottom={"$20"}>
          <Divider mb="$16" />
          <Hero onGetStarted={() => setShowWallet(true)} />
          <Divider mb="$16" />
        </Box>
      )}
    </Layout>
  );
}