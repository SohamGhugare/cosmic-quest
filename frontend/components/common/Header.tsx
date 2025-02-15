import {
  Box,
  Button,
  Icon,
  Link,
  Text,
  useColorModeValue,
  useTheme,
} from "@interchain-ui/react";

const stacks = ["Cosmos Kit", "Next.js", "CosmJS"];

export function Header() {
  const { theme, setTheme } = useTheme();

  const toggleColorMode = () => {
    setTheme(theme === "light" ? "dark" : "light");
  };

  const handleEarlyAccessClick = () => {
    window.location.href = "https://forms.gle/your-google-form-id";
  };

  return (
    <Box marginBottom={"$15"}>
      <Box display="flex" justifyContent="end" mb="$8">
        <Button
          intent="secondary"
          size="sm"
          attributes={{
            paddingX: 0,
            marginX: "$5",
          }}
          onClick={handleEarlyAccessClick}
        >
          Early Access
        </Button>
        <Button
          intent="secondary"
          size="sm"
          attributes={{
            paddingX: 0,
          }}
          onClick={toggleColorMode}
        >
          <Icon name={useColorModeValue("moonLine", "sunLine")} />
        </Button>
      </Box>

      <Box textAlign="center">
        <Text
          as="h1"
          fontWeight="$extrabold"
          fontSize={{ mobile: "$6xl", tablet: "$10xl" }}
          attributes={{
            marginBottom: "$8",
          }}
        >
          Cosmic Quest
        </Text>
        <Text as="h2" fontWeight="$bold">
          <Text
            as="span"
            fontSize={{ mobile: "$3xl", tablet: "$8xl", desktop: "$8xl" }}
          >
            Learn Blockchain By
          </Text>
          <Text
            as="span"
            fontSize={{ mobile: "$3xl", tablet: "$8xl", desktop: "$8xl" }}
            color={useColorModeValue("$purple400", "$primary200")}
          >
            {' Doing '}
          </Text>
          <Text
            as="span"
            fontSize={{ mobile: "$3xl", tablet: "$8xl", desktop: "$8xl" }}
          >
            Blockchain
          </Text>
        </Text>
      </Box>
    </Box>
  );
}
