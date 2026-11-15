package com.editorial.core.util;

import com.editorial.core.exception.ControllerException;
import com.editorial.core.exception.ErrorResponses;

public final class Phase1Guard {

    private Phase1Guard() {
    }

  /** Mutaciones de la API interna se implementan en la fase de autenticación y reglas por área. */
    public static void rejectMutation() throws ControllerException {
        throw new ControllerException(ErrorResponses.NOT_IMPLEMENTED);
    }
}
