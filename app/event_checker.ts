import {Program, web3} from "@coral-xyz/anchor";
import {PriceOracle} from "../target/types/price_oracle"

interface Asset {
    id: string,
    value: number,
    appreciation_rate: number,
    rent: number,
    cumulative_revenue: number,
    cumulative_maintenance_cost: number,
    block: number;
}
export async function fetchEvents(program: Program<PriceOracle>, connection: web3.Connection, assetInfo: web3.PublicKey) {
    let transactionList = await connection.getSignaturesForAddress(assetInfo)
    let signatureList = transactionList.map(transaction => transaction.signature)
    let results = await connection.getParsedTransactions(signatureList, {maxSupportedTransactionVersion: 0})
    return  results.map(x => {
        const metadata = x.meta.logMessages.filter(x => x.includes('value:'))[0]
        const dataString = metadata.substring(13)
        return parseAsset(dataString)
    })
}

function parseAsset(assetString: string): Asset {
    // Remove curly braces and split the string by commas
    const keyValuePairs = assetString.replace(/{|}/g, '').trim().split(', ');

    // Create an empty object to hold the parsed values
    const assetObject: any = {};

    // Iterate over the key-value pairs and populate the object
    keyValuePairs.forEach(pair => {
        const [key, value] = pair.split(': ');
        // Assign the value, parsing numbers where necessary
        assetObject[key.trim()] = isNaN(Number(value)) ? value.trim() : Number(value);
    });

    // Convert the parsed object to an Asset object
    return {
        id: assetObject.id,
        value: assetObject.value,
        appreciation_rate: assetObject.appreciation_rate,
        rent: assetObject.rent,
        cumulative_revenue: assetObject.cumulative_revenue,
        cumulative_maintenance_cost: assetObject.cumulative_maintenance_cost,
        block: assetObject.block
    };
}