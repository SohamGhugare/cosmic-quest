import { Box, Button, Stack, Text, useColorModeValue } from "@interchain-ui/react";
import { useState } from "react";

export function Hero({ onGetStarted }: { onGetStarted: () => void }) {
  return (
    <Box
      display="flex"
      justifyContent="center"
      alignSelf={"center"}
      height="40vh"
      maxWidth={"300px"}
      backgroundColor={useColorModeValue("$white", "$blackAlpha500")}
      padding={{ mobile: "$12", tablet: "$16", desktop: "$8"}}
      borderWidth={ "thin" }
      borderColor={"$black"}
      borderStyle={"$solid"}
      marginY={"$16"}
    
    >
      <Stack
        direction="vertical"
      >
        <Box
            padding={{ mobile: "$8", tablet: "$16", desktop: "$12"}}
        >
            <Text fontSize="$4xl" fontWeight="bold" textAlign={"center"}>
            Genesis Realm
            </Text>
        </Box>
        
        <Box
            padding={{ mobile: "$8", tablet: "$16", desktop: "$8"}}
        >
            <Text fontSize="$xl" textAlign="center" fontWeight={"$bold"}>
            Quest 1
            </Text>
            <Text fontSize="$lg" textAlign="center">
            Create your own Web3 Wallet
            </Text>
        </Box>
        
        <Box
            padding={{ mobile: "$8", tablet: "$16", desktop: "$12"}}
            alignItems={"center"}
            alignSelf={"center"}
        >
            <Button onClick={onGetStarted}>Get Started</Button>
        </Box>
      </Stack>
    </Box>
  );
}