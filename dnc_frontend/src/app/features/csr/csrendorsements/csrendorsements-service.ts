import {inject, Injectable} from '@angular/core';
import {LoginService} from '../../../login.service';
import {environment} from '../../../../environments/environment';
import {HttpClient, HttpHeaders} from '@angular/common/http';
import {Observable} from 'rxjs';

export interface CsrEndorsementResponse {
    id: number;
    hmo_short_name: string;
    company_name: string;
    date_start: string;
    date_end: string;
    benefits: string;
}
@Injectable({
  providedIn: 'root',
})
export class CSREndorsementsService {
    private readonly http = inject(HttpClient);
    private readonly loginService = inject(LoginService);
    private authHeaders(): HttpHeaders {
        const token = this.loginService.token?.() ?? '';
        return new HttpHeaders({ Authorization: `Bearer ${token}` });
    }

    // ANNOTATED CHANGE: adjust this to match your actual route
    private readonly baseUrl = `${environment.apiUrl}/api/csr/endorsements`;

    getEndorsements():Observable<CsrEndorsementResponse[]>{
        return this.http.get<CsrEndorsementResponse[]>(
            this.baseUrl,
            {headers: this.authHeaders(),}
        );
    }

}
