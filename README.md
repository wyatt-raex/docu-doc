# DOCU-DOC

Docu-Doc is a software application developed using Rust to help Windows users sort through cluttered folders without having to do it manually. We've currently employed two different sorting algorithms: extension sort (which sorts files into subfolders based on their extension) and date sort (which sorts files into subfolders by year and then month).

![docu_doc1](https://user-images.githubusercontent.com/75757453/235280309-4a040ae4-bb60-401b-9211-03848f832e46.png)

When you first open Docu-Doc, you will be presented with a GUI that asks for you to select a predetermined path or provide a custom one (done by copy and pasting the directory pathing) in the textbar. Then, you can select your algorithm and click the sort button. ![docu_doc2](https://user-images.githubusercontent.com/75757453/235280421-980fd39f-10fc-49f6-9b46-90732d00e383.png)

After that, Docu-Doc handles the rest for you. It will recurse into subfolders within the main folder too (so be careful where you use it!). Planned features for Docu-Doc in the future include support for Linux and Mac OS, as well as expanded options, more sorting algorithms, and easier specification of custom pathing.
