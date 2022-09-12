# Solana Hiring Tasks

Hi 👋
Congratulations on passing the first round of the Solana Auditor Trainee interview. We have 2 tasks for you.

1. The first task is focused on programming in Rust. The goal is to write efficient, readable and extensible code.
2. The second task is focused on the analysis of the Solana hack. The goal is to write a technical report. Preferred language is English.

You have a maximum of 10 days to finish both tasks. For the final solution to both tasks, please use Github. Good luck🤞.

# Rust 🦀

- **Assignment**
    
    The mysterious device is reproducing a virus. After the first four presses, it creates 1, 2, 3, 5 copies of the virus. With each subsequent click, the viruses multiply, so that the viruses created by the last(*posledním*), second to last(*předposleným*), and second to last(*předpředpředposledním*) click create one copy of themselves. Find out how many viruses are there in the world after Andrej clicks the button X times.
    
    **Input and output:**
    
    In the first line of the input is the number `1<=T<=1000` - it determines the number of questions (number of following lines). In each of the next T lines there is an integer `4<=X<=10^10`. For each X, count how many viruses there are in the world. Since there can be really many, just list its remainder after dividing by 10^9 + 7.
    
    **Example:**
    
    Input
    3
    7
    5
    47
    Output
    64
    20
    349633386
    
    In the fifth click, the viruses from the first, third and fourth clicks multiply, so 1+3+5=9. Together with the remaining eleven, there are 20 viruses.
    
    - Czech version
        
        Tajemné zařízení v sobě množí virus. Po prvních čtyřech stlačeních vytvoří 1, 2, 3, 5 kopií virů. Každým dalším kliknutím se viry rozmnoží, a to tak, že viry vytvořené posledním, předposleným a předpředpředposledním kliknutím vytvoří jednu svou kopii. Zjistěte, kolik je na světě virů poté, co Andrej klikne knoflík X krát.
        **Vstup a výstup**:
        V prvním řádku vstupu je číslo `1<=T<=1000` - určuje počet otázek (počet následujících řádků). V každém z dalších T řádků je celé číslo `4<=X<=10^10`. Pro každé X sečtěte, kolik virů je na světě. Jelikož jich může být fakt hodně, vypište jen jeho zbytek po dělení 10^9 + 7.
        **Příklady:**
        Vstup
        3
        7
        5
        47
        Výstup
        64
        20
        349633386
        V pátém kliknutí se rozmnoží viry z prvního, třetího a čtvrtého, bude jich tedy tedy 1+3+5=9. Spolu se zbývajícími jedenácti jich je tedy 20.
        
    
    [Input](https://s3-us-west-2.amazonaws.com/secure.notion-static.com/44b103da-9f65-44a8-82fa-1b16e144338e/Untitled.txt)
    
    🦀 The preferred implementation language is Rust but C/C++ is also an acceptable choice.
    
    📖 Along with the implementation, we also want a **Readme.md** that describes the solution, or you can mention anything you think is relevant.
    
    ✔️ Final solution push to Github repository.
    

# **Analysis of the Solana hack** 🥷

- **Assignment**
    
    💰 **Introduction** 
    
    During the first 5 months of 2022, DeFi hacks have amounted up to **$1.4 billion** in financial losses. Even though one of the largest hacks was conducted on Solana (Wormhole), Solana's statistics are actually really good compared to other major L1's - both in occurrences and the total amount stolen. Good news: **Over 30 major hacks were conducted since January 2022 and only 3 of them happened on Solana ecosystem.**
    
    **Your task**
    
    Write a report describing [Wormhole Hack](https://twitter.com/wormholecrypto/status/1489001949881978883?ref_src=twsrc%5Etfw%7Ctwcamp%5Etweetembed%7Ctwterm%5E1489001949881978883%7Ctwgr%5E21a12c5a59af6b59a04624118e8a9a24ab8edae8%7Ctwcon%5Es1_&ref_url=https%3A%2F%2Fwww.theverge.com%2F2022%2F2%2F3%2F22916111%2Fwormhole-hack-github-error-325-million-theft-ethereum-solana).
    
    The report should contain following sections:
    
    - General description of what happened
    - Exploit description (step-by-step)
    
    Add any arbitrary sections you think are necessary.
    
    📝 It should be a technical report (similar to a post-mortem). 
    
    📄The form is up to you; it can be Tex, Google docs, MD/ADOC file, or anything else. 
    
    🇬🇧 The preferred language is English.
    
    ✔️ Final report push to Github repository.