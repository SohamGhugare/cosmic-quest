import { useState } from "react";
import { Divider } from "@interchain-ui/react";
import { Layout, Wallet } from "@/components";
import { Hero } from "@/components/Hero";

export default function Home() {
  const [showWallet, setShowWallet] = useState(false);

  return (
    <Layout>
      {showWallet ? (
        <>
          <Wallet />
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