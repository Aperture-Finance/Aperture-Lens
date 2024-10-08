import { z } from "zod";

export const AutomatedMarketMakerEnum = z.enum([
    'UNISWAP_V3',
    'PANCAKESWAP_V3',
    'SLIPSTREAM',
]);
export type AutomatedMarketMakerEnum = z.infer<typeof AutomatedMarketMakerEnum>;

export function ammToSolidityDexEnum(amm: AutomatedMarketMakerEnum): number {
    if (amm === AutomatedMarketMakerEnum.enum.UNISWAP_V3) {
        return 0;
    }
    if (amm === AutomatedMarketMakerEnum.enum.PANCAKESWAP_V3) {
        return 1;
    }
    if (amm === AutomatedMarketMakerEnum.enum.SLIPSTREAM) {
        return 2;
    }
    throw new Error(`Unexpected AMM: ${amm}`);
}
