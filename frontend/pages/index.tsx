import { useState } from "react";
import { useRouter } from "next/router";
import { Divider, Text, Box, Button } from "@interchain-ui/react";
import { Layout, Wallet } from "@/components";
import { Hero } from "@/components/Hero";

export default function Home() {
  const [showWallet, setShowWallet] = useState(false);
  const [walletConnected, setWalletConnected] = useState(false);
  const [loading, setLoading] = useState(false);
  const router = useRouter();

  const handleWalletConnect = () => {
    setLoading(true);
    setTimeout(() => {
      setWalletConnected(true);
      setLoading(false);
    }, 2000); // Simulate a 2-second delay
  };

  const handleClaimNft = () => {
    router.push("/nft");
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
              <>
                <Text fontSize="$lg" color="green" textAlign={"center"}>
                  Congrats! You just completed your first quest!
                </Text>
                <Box display="flex" justifyContent="center" mt="$10">
                  <Button onClick={handleClaimNft}>
                    Claim my NFT
                  </Button>
                </Box>
              </>
            )}
          </Box>
          <Divider mb="$16" />
        </>
      ) : (
        <>
          <Divider mb="$16" />
          <Hero onGetStarted={() => setShowWallet(true)} />
          <Divider mb="$16" />
        </>
      )}
    </Layout>
  );
}