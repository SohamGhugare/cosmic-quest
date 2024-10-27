import { Layout } from "@/components";
import { Box, NftProfileCard, Text } from "@interchain-ui/react";

export default function NftPage() {
  return (
    <Layout>
        <Box maxWidth="360px" mx="auto" textAlign="center" mt="$10">
            <Box my={"$10"}>
                <Text fontSize="$2xl" fontWeight="normal" color={"$purple700"}>
                    Your first NFT, cool innit?
                </Text>
            </Box>
            
            <NftProfileCard
                imgSrc="nft.png"
                name="Genesis Explorer (Epic)"
                
                onClick={() => {
                alert("NFT Clicked");
                }}
            />
        </Box>

    </Layout>
  );
}