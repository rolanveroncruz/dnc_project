import { Injectable, inject } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable, shareReplay } from 'rxjs';

import { environment } from '../../environments/environment';

export interface AccountType {
    id: number;
    name: string;
}

@Injectable({ providedIn: 'root' })
export class AccountTypeService {
    private readonly http = inject(HttpClient);

    // ANNOTATED CHANGE: adjust this to match your actual route
    private readonly baseUrl = `${environment.apiUrl}/api/account_types`;

    /**
     * GET /api/account_types
     */
    getAllAccountTypes(): Observable<AccountType[]> {
        return this.http
            .get<AccountType[]>(this.baseUrl)
            // cache latest value for multiple subscribers
            .pipe(shareReplay({ bufferSize: 1, refCount: true }));
    }
}
