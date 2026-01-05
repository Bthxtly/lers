BEGIN
    BEGIN
        number := 12;
        a := number;
        b := 10 * a + 10 * number / 16;
        c := a - - b
    END;
    x := 11;
END.
