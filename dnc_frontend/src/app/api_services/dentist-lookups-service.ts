import { Injectable,inject } from '@angular/core';
import {HttpClient, HttpHeaders} from '@angular/common/http';
import { Observable } from 'rxjs';
import {LoginService} from '../login.service';
import {environment} from '../../environments/environment';


export interface DentistHistory {
    id: number;
    name: string;
}
export interface DentistStatus {
    id: number;
    name: string;
}
export interface TaxClassification {
    id: number;
    name: string;
}
export interface TaxType {
    id: number;
    name: string;
}

@Injectable({
  providedIn: 'root',
})
export class DentistLookupsService {
    private readonly http = inject(HttpClient);
    private readonly loginService = inject(LoginService);
    // Adjust if you store this elsewhere (e.g. environment.apiBaseUrl)
    private readonly baseUrl = `${environment.apiUrl}/api`;
    private authHeaders(): HttpHeaders {
        const token = this.loginService.token?.() ?? '';
        return new HttpHeaders({ Authorization: `Bearer ${token}` });
    }

    getAllDentistHistories(): Observable<DentistHistory[]> {
        return this.http.get<DentistHistory[]>(`${this.baseUrl}/dentist_histories/`, { headers: this.authHeaders() });
    }

    getAllDentistStatuses(): Observable<DentistStatus[]> {
        return this.http.get<DentistStatus[]>(`${this.baseUrl}/dentist_statuses/`, { headers: this.authHeaders() });
    }
    getAllTaxClassifications(): Observable<TaxClassification[]> {
        return this.http.get<TaxClassification[]>(`${this.baseUrl}/tax_classifications/`, { headers: this.authHeaders() });
    }
    getAllTaxTypes(): Observable<TaxType[]> {
        return this.http.get<TaxType[]>(`${this.baseUrl}/tax_types/`, { headers: this.authHeaders() });
    }

}
