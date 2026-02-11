import { Injectable, inject } from '@angular/core';
import {HttpClient, HttpHeaders} from '@angular/common/http';
import { Observable, shareReplay } from 'rxjs';

import { environment } from '../../environments/environment';
import {LoginService} from '../login.service';

export interface AccountType {
    id: number;
    name: string;
}

@Injectable({ providedIn: 'root' })
export class AccountTypeService {
    private readonly http = inject(HttpClient);
    private readonly loginService = inject(LoginService);
    private authHeaders(): HttpHeaders {
        const token = this.loginService.token?.() ?? '';
        return new HttpHeaders({ Authorization: `Bearer ${token}` });
    }

    // ANNOTATED CHANGE: adjust this to match your actual route
    private readonly baseUrl = `${environment.apiUrl}/api/bank_account_types`;

    /**
     * GET /api/account_types
     */
    getAllAccountTypes(): Observable<AccountType[]> {
        return this.http
            .get<AccountType[]>(this.baseUrl, { headers: this.authHeaders() })
            // cache latest value for multiple subscribers
            .pipe(shareReplay({ bufferSize: 1, refCount: true }));
    }
}
