// src/app/api_services/provinces-service.ts
import { Injectable, inject } from '@angular/core';
import { HttpClient, HttpHeaders, HttpParams } from '@angular/common/http';
import { Observable } from 'rxjs';

// Adjust this import path to your project
import { environment } from '../../environments/environment';
import { LoginService } from '../login.service';

//
// ---- Types (mirror your Rust handler structs + SeaORM Models)
//

// CHANGE: Added ListQuery to match Rust `ListQuery { page, page_size }`
export interface ListQuery {
  page?: number;       // 1-based
  page_size?: number;  // server clamps
}

// CHANGE: Added PageResponse to match Rust `PageResponse<T>`
export interface PageResponse<T> {
  page: number;       // 1-based
  page_size: number;
  total: number;
  items: T[];
}

// NOTE: Keep these aligned with your actual SeaORM Models.
export interface Province {
  id: number;
  name: string;
  region_id: number; // province::Column::RegionId
}

export interface City {
  id: number;
  name: string;
  province_id: number; // city::Column::ProvinceId
}

// CHANGE: ProvinceListQuery now includes paging fields (flattened in Rust)
export interface ProvinceListQuery extends ListQuery {
  region_id?: number | null; // /provinces?region_id=1
}

@Injectable({ providedIn: 'root' })
export class ProvincesService {
  private readonly http = inject(HttpClient);
  private readonly loginService = inject(LoginService);

  // If your backend uses something like `${environment.apiUrl}/api`, change this accordingly.
  private readonly baseUrl = `${environment.apiUrl}/api/provinces`;

  private authHeaders(): HttpHeaders {
    const token = this.loginService.token?.() ?? '';
    return new HttpHeaders({ Authorization: `Bearer ${token}` });
  }

  /**
   * GET /provinces
   * Supports paging + optional filter:
   *   /provinces?page=1&page_size=650&region_id=1
   */
  // CHANGE: Return type is now PageResponse<Province> to match Rust handler:
  //   Result<Json<PageResponse<province::Model>>, StatusCode>
  getProvinces(query: ProvinceListQuery = {}): Observable<PageResponse<Province>> {
    let params = new HttpParams();

    // CHANGE: Added paging params to match Rust handler inputs
    if (query.page !== undefined && query.page !== null) {
      params = params.set('page', String(query.page));
    }
    if (query.page_size !== undefined && query.page_size !== null) {
      params = params.set('page_size', String(query.page_size));
    }

    // Existing filter param (still valid)
    if (query.region_id !== undefined && query.region_id !== null) {
      params = params.set('region_id', String(query.region_id));
    }

    // CHANGE: Backend returns PageResponse<Province>, not Province[]
    return this.http.get<PageResponse<Province>>(this.baseUrl, {
      params,
      headers: this.authHeaders(),
    });
  }

  /**
   * GET /provinces/:province_id/cities
   * Rust handler returns 404 if province does not exist.
   */
  getCitiesByProvince(provinceId: number): Observable<City[]> {
    // CHANGE: Added auth headers here too (your backend likely expects it)
    return this.http.get<City[]>(`${this.baseUrl}/${provinceId}/cities`, {
      headers: this.authHeaders(),
    });
  }
}
