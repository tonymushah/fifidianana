import { Box, Divider, HStack, Link, Spacer } from "@chakra-ui/react";
import { NavLink, useLocation } from "@remix-run/react";
import React from "react";
import { ColorModeSwitcher } from "./ColorModeButton";

export default function RootNavBar(){
    const location = useLocation();
    return (
        <React.Fragment>
            <HStack spacing={4}>
                <Link color={location.pathname == "/" ? "red.500" : undefined} marginLeft={4} as={NavLink} to={"/"}>Home</Link>
                <Spacer/>
                <Link as={NavLink} color={location.pathname.startsWith("/article") ? "red.500" : undefined} to={"/article"}>Articles</Link>
                <Link as={NavLink} color={location.pathname.startsWith("/magasin") ? "red.500" : undefined} to={"/magasin"}>Magasin</Link>
                <Link as={NavLink} color={location.pathname.startsWith("/unite-mesure") ? "red.500" : undefined} to={"/unite-mesure"}>Unite de mesure</Link>
                <Link as={NavLink} color={location.pathname.startsWith("/etat-stock") ? "red.500" : undefined} to={"/etat-stock"}>Etat du Stock</Link>
                <Link as={NavLink} color={location.pathname.startsWith("/mouvement") ? "red.500" : undefined} to={"/mouvement"}>Mouvement</Link>
                <Box m={1}>
                    <ColorModeSwitcher/>
                </Box>
            </HStack>
            <Divider/>
        </React.Fragment>
    )
}