#[macro_export]
macro_rules! mock_peripheral {
    ($($key:tt : $val:expr),* $(,)?) => { 
        {
            use std::{collections::BTreeSet, pin::Pin};

            use async_trait::async_trait;
            use futures::Stream;

            use crate::{api::{BDAddr, Characteristic, Descriptor, Peripheral, PeripheralProperties, Service, ValueNotification, WriteType}, platform::PeripheralId};
            use crate::Result;

            #[derive(Debug, Clone)]
            struct MockPeripheral {}

            #[async_trait]
            impl Peripheral for MockPeripheral {
                fn id(&self) -> PeripheralId {
                    panic!();
                }

                fn address(&self) -> BDAddr {
                    panic!();
                }

                async fn properties(&self) -> Result<Option<PeripheralProperties>> {
                }

                fn services(&self) -> BTreeSet<Service> {
                }

                async fn is_connected(&self) -> Result<bool> {
                }

                async fn connect(&self) -> Result<()> {
                }

                async fn disconnect(&self) -> Result<()> {
                }

                async fn discover_services(&self) -> Result<()> {
                }

                async fn write(
                    &self,
                    characteristic: &Characteristic,
                    data: &[u8],
                    write_type: WriteType,
                ) -> Result<()> {
                }

                async fn read(&self, characteristic: &Characteristic) -> Result<Vec<u8>> {
                }

                async fn subscribe(&self, characteristic: &Characteristic) -> Result<()> {
                }

                async fn unsubscribe(&self, characteristic: &Characteristic) -> Result<()> {
                }

                async fn notifications(&self) -> Result<Pin<Box<dyn Stream<Item = ValueNotification> + Send>>> {
                }

                async fn write_descriptor(&self, descriptor: &Descriptor, data: &[u8]) -> Result<()> {
                }

                async fn read_descriptor(&self, descriptor: &Descriptor) -> Result<Vec<u8>> {
                }
            }

            MockPeripheral{}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mock_peripheral;

    #[test]
    fn test_add() {

        let peripheral = mock_peripheral!{
            address: "test"
        };
    }
}
