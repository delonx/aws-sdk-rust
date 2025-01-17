/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_iotsitewise::{Client, Error, Region, model::ListAssetsFilter, PKG_VERSION};
use aws_smithy_types_convert::date_time::DateTimeExt;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// The AWS Region.
    #[structopt(short, long)]
    region: Option<String>,

    /// The asset model id.
    #[structopt(short, long)]
    asset_model_id: Option<String>,

    /// The filter.
    #[structopt(short, long)]
    filter: String,

    /// Whether to display additional information.
    #[structopt(short, long)]
    verbose: bool,
}

// Displays the address of an endpoint.
// snippet-start:[iot.rust.describe-endpoint]
async fn list_assets(client: &Client, filter: ListAssetsFilter, asset_model_id: Option<String>) -> Result<(), Error> {  
    let resp = client.list_assets()
        .filter(filter)
        .set_asset_model_id(asset_model_id)
        .send()
        .await?;

    println!("Assets:");

    for asset in resp.asset_summaries.unwrap() {
        println!("  ID:  {}", asset.id.as_deref().unwrap_or_default());
        println!("  ARN:  {}", asset.arn.as_deref().unwrap_or_default());
        println!("  Name:   {}", asset.name.as_deref().unwrap_or_default());
        println!("  Asset Model ID:   {}", asset.asset_model_id.as_deref().unwrap_or_default());
        println!("  Creation Date:   {}", asset.creation_date.unwrap().to_chrono_utc());
        println!("  Last Update Date:   {}", asset.last_update_date.unwrap().to_chrono_utc());
        println!("  Current Status:   {}", asset.status.unwrap().state.unwrap().as_str());

        println!("  Assets Hierarchies:");

        for hierarchy in asset.hierarchies.unwrap() {
            println!("    Hierarchy ID:   {}", hierarchy.id.as_deref().unwrap_or_default());
            println!("    Hierarchy Name:   {}", hierarchy.name.as_deref().unwrap_or_default());
        }

        println!();
    }

    println!();

    Ok(())
}
// snippet-end:[sitewise.rust.list-assets]

/// Lists the id, arn, name, asset_model_id, creation_date, last_update_data, status
/// and hierarchies of your Sitewise Assets in the Region.
///
/// # Arguments
///
/// * `-f FILTER` - The type of filter.
///   Must be one of:
///   - ALL - The list includes all assets for a given asset model ID. The assetModelId parameter
///   is required if you filter by ALL.
///   - TOP_LEVEL - The list includes only top-level assets in the asset hierarchy tree.
/// * `[-a ASSET-MODEL-ID]` - The ID of the asset model by which to filter the list of assets.
///   This parameter is required if you choose ALL for filter.
/// * `[-r REGION]` - The Region in which the client is created.
///   If not supplied, uses the value of the **AWS_REGION** environment variable.
///   If the environment variable is not set, defaults to **us-west-2**.
/// * `[-v]` - Whether to display information.
#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    let Opt { 
        region,
        asset_model_id,
        filter,
        verbose,
    } = Opt::from_args();

    let region_provider = RegionProviderChain::first_try(region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));
    println!();

    if verbose {
        println!("IoT client version: {}", PKG_VERSION);
        println!(
            "Region:             {}",
            region_provider.region().await.unwrap().as_ref()
        );
        println!("Filter:      {}", &filter);
        println!();
    }

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);
    let filter : ListAssetsFilter = ListAssetsFilter::from(filter.as_str());

    list_assets(&client, filter, asset_model_id).await
}
