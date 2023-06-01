# UEFI

## Boot Stage
1. Platform Initialization
uefi crate ではスコープ外

2. Boot Services
BootServices と RuntimeServices テーブルの両方にアクセス可能  
通常 OS の bootloader はこのステージで実行される
このステージが終了すると Runtime モードに移行する

3. Runtime
通常 OS を実行しているときにアクティブになる  
BootService テーブルにアクセスできなくなる  
システムがリセットされるまで、Boot Services ステージに戻ることができない

## Tables
* SystemTable
仕様では EFI_SYSTEM_TABLE  
他のテーブルを提供  

* BootServices
仕様では EFI_BOOT_SERVICES  
Boot Services ステージでのみ使用可能  
メモリの割当、実行可能ファイルの読み込み、各種プロトコルのインターフェースを提供  

* RuntimeServices
仕様では EFI_RUNTIME_SERVICES  
Boot Services と Runtime Services ステージで使用可能  
変数ストレージ、システム時間、仮想メモリマップを提供  

## GUID
Global Unique Identifier の略  
313b0d7c-fed4-4de7-99ed-2fe48874a410 のような 16 byte で表現される  
数値のフォーマットはあまり重要ではないが、先頭３フィールドはリトルエンディアンで表現されている  
プロトコル、ディスクパーティション、変数グループなどを識別するために使用される  

## Handles and Protocols
* Handles
HDD, USB などの物理デバイス、実行可能ファイルなどのリソースを表す
Handles はポインタなので、直接操作することができない
Handles を操作するには Protocols を開く必要がある

* Protocols
Handles で取得したリソースのインターフェース
例えば BlockIO は読み書き可能な boloc IO デバイスのインターフェース
Protocols は Boot Services ステージでのみ取得可能

## Device Paths

