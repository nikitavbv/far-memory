use crate::thesis::engine::{Block, paragraph, subsection_header, reference, Reference};

/*
latency numbers:
https://www.naddod.com/blog/differences-between-infiniband-and-ethernet-networks
*/
pub fn rdma() -> Block {
    Block::Multiple(vec![
        subsection_header("Remote Direct Memory Access та її реалізації"),
        paragraph(
            "Технологія віддаленого прямого доступу до памʼяті (Remote Direct Memory Access) полягає в використанні спеціальних апаратних засобів, \
що дозволяють вузлам в системі отримувати дані з невеликою затримкою з інших вузлів без витрачання ресурсів процесору цих вузлів для обробки \
запитів."),
        paragraph(vec![
            "Однією з найбільш розповсюджених реалізацій RDMA є ".into(),
            reference("InfiniBand", Reference::for_website(
                "Understanding InfiniBand and RDMA // Red Hat Customer Portal".to_owned(),
                "https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/8/html/configuring_infiniband_and_rdma_networks/understanding-infiniband-and-rdma_configuring-infiniband-and-rdma-networks".to_owned()
            )),
            ". Ця реалізація забезпечує \
пропускну здатність до 50Гбіт/c на кожен кабель, при цьому має затримку що є в десятки раз меншою (в залежності \
від розміру повідомлення) ніж за умов пересилання даних по ".into(),
            reference("TCP", Reference::for_website("RFC 9293 - Transmission Control Protocol", "https://datatracker.ietf.org/doc/html/rfc9293")),
            " через ".into(),
            reference("Gigabit Ethernet", Reference::for_website("IEEE 802.3 ETHERNET", "https://www.ieee802.org/3/")),
            ".".into()
        ]),
        paragraph("RDMA використовується і є виправданим для використання в середовищах \
високопродуктивних обчислень (High Performance Computing - HPC). Завдяки низькій затримці, програмне забезпечення \
може ефективно працювати з наборами даних, які розподілені у оперативній памʼяті багатьох вузлів."),
        paragraph(vec![
            "Головним недоліком цієї реалізації віддаленої памʼяті є те, що вона потребує додаткового спеціалізованого обладнання. Для задач та \
середовища що розглядаються в цій роботі не є підхожим рішенням, тому що використання додаткових пристроїв потребує \
додаткових ресурсів і не вирішує проблему більш ефективного використання наявних ресурсів без змін в апаратну платформу. Крім цього, RDMA вирішує задачу \
передачі даних з низькою затримкою між вузлами, а забезпечення відмовостійкості та керування переміщенням даних між локальною памʼяттю та памʼяттю \
віддалених вузлів забезпечується розробником прикладного програмного забезпечення. Для інтеграції у програмне забезпечення зазвичай потрібні значні \
зміни, наприклад використання ".into(),
        reference("MPI (Message Passing Interface)", Reference::for_website("MPI Forum", "https://www.mpi-forum.org/")),
        ".".into()]),
    ])
}
