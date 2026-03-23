// src/app/api_services/dentist-company-relations.service.ts
import { Injectable, inject } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import {map, Observable} from 'rxjs';
import { LoginService } from '../login.service';
import { environment } from '../../environments/environment';

export interface CompanyListItem {
    id: number;
    short_name: string;
}
interface EndorsementCompanyItem{
    id: number;
    name: string;
}
@Injectable({ providedIn: 'root' })
export class DentistCompanyRelationsService {
    private readonly http = inject(HttpClient);
    private readonly loginService = inject(LoginService);

    private readonly baseUrl = `${environment.apiUrl}/api`;

    private authHeaders(): HttpHeaders {
        const token = this.loginService.token?.() ?? '';
        return new HttpHeaders({ Authorization: `Bearer ${token}` });
    }

    getAllCompanies(): Observable<CompanyListItem[]> {
        return this.http.get<EndorsementCompanyItem[]>(
            `${this.baseUrl}/endorsements/companies`,
            { headers: this.authHeaders() }
        ).pipe(
            map((items)=>items.map((item)=>({
                id: item.id,
                short_name: item.name,
            })))
        );
    }

    /** GET /dentists/:dentist_id/companies/exclusive */
    getExclusiveToCompanies(dentistId: number): Observable<CompanyListItem[]> {
        return this.http.get<CompanyListItem[]>(
            `${this.baseUrl}/dentists/${dentistId}/companies/exclusive`,
            { headers: this.authHeaders() }
        );
    }

    /** POST /dentists/:dentist_id/companies/exclusive/:company_id */
    addExclusiveToCompanies(dentistId: number, companyId: number): Observable<any> {
        return this.http.post<any>(
            `${this.baseUrl}/dentists/${dentistId}/companies/exclusive/${encodeURIComponent(companyId)}`,
            { company_id: companyId },
            { headers: this.authHeaders() }
        );
    }

    /** DELETE /dentists/:dentist_id/companies/exclusive/:company_id */
    removeExclusiveToCompanies(dentistId: number, companyId: number): Observable<any> {
        return this.http.delete<any>(
            `${this.baseUrl}/dentists/${dentistId}/companies/exclusive/${encodeURIComponent(companyId)}`,
            { headers: this.authHeaders() }
        );
    }

    /** GET /dentists/:dentist_id/companies/except */
    getExceptForCompanies(dentistId: number): Observable<CompanyListItem[]> {
        return this.http.get<CompanyListItem[]>(
            `${this.baseUrl}/dentists/${dentistId}/companies/except`,
            { headers: this.authHeaders() }
        );
    }

    /** POST /dentists/:dentist_id/companies/except/:company_id */
    addExceptForCompanies(dentistId: number, companyId: number): Observable<any> {
        return this.http.post<any>(
            `${this.baseUrl}/dentists/${dentistId}/companies/except/${encodeURIComponent(companyId)}`,
            { company_id: companyId },
            { headers: this.authHeaders() }
        );
    }

    /** DELETE /dentists/:dentist_id/companies/except/:company_id */
    removeExceptForCompanies(dentistId: number, companyId: number): Observable<any> {
        return this.http.delete<any>(
            `${this.baseUrl}/dentists/${dentistId}/companies/except/${encodeURIComponent(companyId)}`,
            { headers: this.authHeaders() }
        );
    }
}
