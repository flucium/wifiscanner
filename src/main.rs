use futures::executor::block_on;
use futures::future::join;
use windows::core::*;
use windows::Devices::WiFi;

//
fn main() {
    
    // WiFi::WiFiAdapter::FromIdAsync(WiFi::WiFiAdapter::FindAllAdaptersAsync().unwrap());
    
    
    block_on(async_main());
}

async fn async_main() {
    
    async {
        let mut a= WiFi::WiFiAdapter::FindAllAdaptersAsync().unwrap().get().unwrap();    
        // println!("{:?}",a.First().unwrap().Current().unwrap().ScanAsync().unwrap());        
        println!("{:?}",a.First().unwrap().next().unwrap().ScanAsync().unwrap());
        // a.First().unwrap().next().unwrap().NetworkReport().unwrap().AvailableNetworks().unwrap().First().unwrap().next().unwrap().Ssid()
        // println!("{:?}",a.First().unwrap().next().unwrap().NetworkReport().unwrap().AvailableNetworks().unwrap().First().unwrap().next().unwrap().Ssid());
        let mut ns=a.First().unwrap().next().unwrap().NetworkReport().unwrap().AvailableNetworks().unwrap().First().unwrap();

        while let Some(n) = ns.next(){
            println!("{:?}",n.SignalBars().unwrap());
            println!("{:?}",n.Bssid().unwrap());
            println!("{:?}",n.Ssid().unwrap());
            println!("{:?}",n.IsWiFiDirect().unwrap());
            println!("{:?}",n.NetworkRssiInDecibelMilliwatts().unwrap());
            println!("-------------------------\n\n");
        }
        
    }
    .await;
}
